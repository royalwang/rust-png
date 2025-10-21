import Stripe from 'stripe'
import { logger } from '../utils/logger'
import { CustomError } from '../middleware/errorHandler'
import { StatusCodes } from 'http-status-codes'

export interface CreateCustomerData {
  email: string
  name: string
  userId: string
}

export interface CreateSubscriptionData {
  customerId: string
  priceId: string
  paymentMethodId?: string
}

export interface UpdateSubscriptionData {
  subscriptionId: string
  priceId?: string
  paymentMethodId?: string
}

export class StripeService {
  private stripe: Stripe

  constructor() {
    this.stripe = new Stripe(process.env.STRIPE_SECRET_KEY!, {
      apiVersion: '2023-10-16'
    })
  }

  async createCustomer(data: CreateCustomerData) {
    try {
      const customer = await this.stripe.customers.create({
        email: data.email,
        name: data.name,
        metadata: {
          userId: data.userId
        }
      })

      logger.info(`Stripe客户创建成功: ${customer.id} (用户: ${data.userId})`)
      return customer
    } catch (error) {
      logger.error('Stripe客户创建失败', error)
      throw new CustomError('客户创建失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getCustomer(customerId: string) {
    try {
      const customer = await this.stripe.customers.retrieve(customerId)
      return customer
    } catch (error) {
      logger.error('获取Stripe客户失败', error)
      throw new CustomError('获取客户信息失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async updateCustomer(customerId: string, data: Partial<CreateCustomerData>) {
    try {
      const customer = await this.stripe.customers.update(customerId, {
        email: data.email,
        name: data.name
      })

      logger.info(`Stripe客户更新成功: ${customerId}`)
      return customer
    } catch (error) {
      logger.error('Stripe客户更新失败', error)
      throw new CustomError('客户更新失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async deleteCustomer(customerId: string) {
    try {
      await this.stripe.customers.del(customerId)
      logger.info(`Stripe客户删除成功: ${customerId}`)
    } catch (error) {
      logger.error('Stripe客户删除失败', error)
      throw new CustomError('客户删除失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async createSubscription(data: CreateSubscriptionData) {
    try {
      const subscriptionData: Stripe.SubscriptionCreateParams = {
        customer: data.customerId,
        items: [{ price: data.priceId }],
        payment_behavior: 'default_incomplete',
        payment_settings: { save_default_payment_method: 'on_subscription' },
        expand: ['latest_invoice.payment_intent']
      }

      if (data.paymentMethodId) {
        subscriptionData.default_payment_method = data.paymentMethodId
      }

      const subscription = await this.stripe.subscriptions.create(subscriptionData)

      logger.info(`Stripe订阅创建成功: ${subscription.id}`)
      return subscription
    } catch (error) {
      logger.error('Stripe订阅创建失败', error)
      throw new CustomError('订阅创建失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getSubscription(subscriptionId: string) {
    try {
      const subscription = await this.stripe.subscriptions.retrieve(subscriptionId)
      return subscription
    } catch (error) {
      logger.error('获取Stripe订阅失败', error)
      throw new CustomError('获取订阅信息失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async updateSubscription(data: UpdateSubscriptionData) {
    try {
      const updateData: Stripe.SubscriptionUpdateParams = {}

      if (data.priceId) {
        updateData.items = [{
          id: (await this.stripe.subscriptions.retrieve(data.subscriptionId)).items.data[0].id,
          price: data.priceId
        }]
      }

      if (data.paymentMethodId) {
        updateData.default_payment_method = data.paymentMethodId
      }

      const subscription = await this.stripe.subscriptions.update(
        data.subscriptionId,
        updateData
      )

      logger.info(`Stripe订阅更新成功: ${data.subscriptionId}`)
      return subscription
    } catch (error) {
      logger.error('Stripe订阅更新失败', error)
      throw new CustomError('订阅更新失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async cancelSubscription(subscriptionId: string, immediately: boolean = false) {
    try {
      let subscription: Stripe.Subscription

      if (immediately) {
        subscription = await this.stripe.subscriptions.cancel(subscriptionId)
      } else {
        subscription = await this.stripe.subscriptions.update(subscriptionId, {
          cancel_at_period_end: true
        })
      }

      logger.info(`Stripe订阅取消成功: ${subscriptionId}`)
      return subscription
    } catch (error) {
      logger.error('Stripe订阅取消失败', error)
      throw new CustomError('订阅取消失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async resumeSubscription(subscriptionId: string) {
    try {
      const subscription = await this.stripe.subscriptions.update(subscriptionId, {
        cancel_at_period_end: false
      })

      logger.info(`Stripe订阅恢复成功: ${subscriptionId}`)
      return subscription
    } catch (error) {
      logger.error('Stripe订阅恢复失败', error)
      throw new CustomError('订阅恢复失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async createPaymentIntent(amount: number, currency: string = 'usd', customerId?: string) {
    try {
      const paymentIntent = await this.stripe.paymentIntents.create({
        amount,
        currency,
        customer: customerId,
        automatic_payment_methods: {
          enabled: true
        }
      })

      logger.info(`Stripe支付意图创建成功: ${paymentIntent.id}`)
      return paymentIntent
    } catch (error) {
      logger.error('Stripe支付意图创建失败', error)
      throw new CustomError('支付意图创建失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getPaymentMethods(customerId: string) {
    try {
      const paymentMethods = await this.stripe.paymentMethods.list({
        customer: customerId,
        type: 'card'
      })

      return paymentMethods.data
    } catch (error) {
      logger.error('获取Stripe支付方式失败', error)
      throw new CustomError('获取支付方式失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async attachPaymentMethod(paymentMethodId: string, customerId: string) {
    try {
      const paymentMethod = await this.stripe.paymentMethods.attach(paymentMethodId, {
        customer: customerId
      })

      logger.info(`Stripe支付方式附加成功: ${paymentMethodId}`)
      return paymentMethod
    } catch (error) {
      logger.error('Stripe支付方式附加失败', error)
      throw new CustomError('支付方式附加失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async detachPaymentMethod(paymentMethodId: string) {
    try {
      const paymentMethod = await this.stripe.paymentMethods.detach(paymentMethodId)
      logger.info(`Stripe支付方式分离成功: ${paymentMethodId}`)
      return paymentMethod
    } catch (error) {
      logger.error('Stripe支付方式分离失败', error)
      throw new CustomError('支付方式分离失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getInvoices(customerId: string, limit: number = 10) {
    try {
      const invoices = await this.stripe.invoices.list({
        customer: customerId,
        limit
      })

      return invoices.data
    } catch (error) {
      logger.error('获取Stripe发票失败', error)
      throw new CustomError('获取发票失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getInvoice(invoiceId: string) {
    try {
      const invoice = await this.stripe.invoices.retrieve(invoiceId)
      return invoice
    } catch (error) {
      logger.error('获取Stripe发票详情失败', error)
      throw new CustomError('获取发票详情失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async createSetupIntent(customerId: string) {
    try {
      const setupIntent = await this.stripe.setupIntents.create({
        customer: customerId,
        payment_method_types: ['card']
      })

      logger.info(`Stripe设置意图创建成功: ${setupIntent.id}`)
      return setupIntent
    } catch (error) {
      logger.error('Stripe设置意图创建失败', error)
      throw new CustomError('设置意图创建失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async constructWebhookEvent(payload: string, signature: string) {
    try {
      const event = this.stripe.webhooks.constructEvent(
        payload,
        signature,
        process.env.STRIPE_WEBHOOK_SECRET!
      )

      return event
    } catch (error) {
      logger.error('Stripe Webhook验证失败', error)
      throw new CustomError('Webhook验证失败', StatusCodes.BAD_REQUEST)
    }
  }

  async getProducts(active: boolean = true) {
    try {
      const products = await this.stripe.products.list({
        active
      })

      return products.data
    } catch (error) {
      logger.error('获取Stripe产品失败', error)
      throw new CustomError('获取产品失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getPrices(productId?: string) {
    try {
      const prices = await this.stripe.prices.list({
        product: productId,
        active: true
      })

      return prices.data
    } catch (error) {
      logger.error('获取Stripe价格失败', error)
      throw new CustomError('获取价格失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }
}
