<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddLineItem</name>
   <tag></tag>
   <elementGuidId>6047a30e-b0fb-48b3-a266-10483ac729b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>pW3L9ZlT78HLaPm4smMbh2Cnw6ZVU0Em</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${GlobalVariable.cart_version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;addLineItem\&quot;,\n            \&quot;sku\&quot;     :\&quot;SCM-02\&quot;,\n            \&quot;quantity\&quot; : 1\n\t\t\t},\n         {\n            \&quot;action\&quot; : \&quot;addLineItem\&quot;,\n            \&quot;sku\&quot;     :\&quot;VC-01\&quot;,\n            \&quot;quantity\&quot; : 1\n         },\n         {\n            \&quot;action\&quot; : \&quot;addLineItem\&quot;,\n            \&quot;sku\&quot;     :\&quot;CDG-09\&quot;,\n            \&quot;quantity\&quot; : 1\n         }\n    ]       \n\n}\n\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>246731e0-059e-452a-9825-a304d54b19a3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.ctp_access_token}</value>
      <webElementGuid>4bdb9ba8-af0d-4bc9-966d-a170e6b144f0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/${GlobalVariable.project_key}/carts/${GlobalVariable.cart_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>31b9ed69-27bc-4e40-b1f8-76f942e9d2b4</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project_key</defaultValue>
      <description></description>
      <id>43cb9642-d1db-45dc-9a8d-6295068a4d5c</id>
      <masked>false</masked>
      <name>project_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_id</defaultValue>
      <description></description>
      <id>25f91e61-1ec2-44e1-b5c7-bb87a78b9e0d</id>
      <masked>false</masked>
      <name>cart_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_version</defaultValue>
      <description></description>
      <id>1df10ff0-a546-4e2e-a964-b0b530808be7</id>
      <masked>false</masked>
      <name>cart_version</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'lineItems[0].variant.sku', &quot;SCM-02&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
